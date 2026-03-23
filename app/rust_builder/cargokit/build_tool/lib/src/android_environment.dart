/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';
import 'dart:math' as math;

import 'package:collection/collection.dart';
import 'package:path/path.dart' as path;
import 'package:version/version.dart';

import 'target.dart';
import 'util.dart';

class AndroidEnvironment {
  AndroidEnvironment({
    required this.sdkPath,
    required this.ndkVersion,
    required this.minSdkVersion,
    required this.targetTempDir,
    required this.target,
  });

  static void clangLinkerWrapper(List<String> args) {
    final clang = Platform.environment['_CARGOKIT_NDK_LINK_CLANG'];
    if (clang == null) {
      throw Exception(
          "cargo-ndk rustc linker: didn't find _CARGOKIT_NDK_LINK_CLANG env var");
    }
    final target = Platform.environment['_CARGOKIT_NDK_LINK_TARGET'];
    if (target == null) {
      throw Exception(
          "cargo-ndk rustc linker: didn't find _CARGOKIT_NDK_LINK_TARGET env var");
    }

    runCommand(clang, [
      target,
      ...args,
    ]);
  }

  /// Loads key=value pairs from `app/.env` (next to pubspec.yaml).
  /// Keys: ANDROID_SDK_ROOT, ANDROID_NDK_VERSION (both optional).
  static Map<String, String> _loadDotEnv() {
    // Platform.script points to the compiled .dill inside the build dir.
    // Walk up to find the app directory (where pubspec.yaml lives).
    var dir = path.dirname(Platform.script.toFilePath());
    for (var i = 0; i < 8; i++) {
      final pubspec = File(path.join(dir, 'pubspec.yaml'));
      if (pubspec.existsSync()) break;
      dir = path.dirname(dir);
    }
    final envFile = File(path.join(dir, '.env'));
    final result = <String, String>{};
    if (!envFile.existsSync()) return result;
    for (final line in envFile.readAsLinesSync()) {
      final trimmed = line.trim();
      if (trimmed.isEmpty || trimmed.startsWith('#')) continue;
      final idx = trimmed.indexOf('=');
      if (idx == -1) continue;
      final k = trimmed.substring(0, idx).trim();
      final v = trimmed.substring(idx + 1).trim();
      if (k.isNotEmpty && v.isNotEmpty) result[k] = v;
    }
    return result;
  }

  /// Full path to Android SDK.
  final String sdkPath;

  /// Full version of Android NDK.
  final String ndkVersion;

  /// Minimum supported SDK version.
  final int minSdkVersion;

  /// Target directory for build artifacts.
  final String targetTempDir;

  /// Target being built.
  final Target target;

  bool ndkIsInstalled() {
    final ndkPath = path.join(sdkPath, 'ndk', ndkVersion);
    final ndkPackageXml = File(path.join(ndkPath, 'package.xml'));
    return ndkPackageXml.existsSync();
  }

  void installNdk({
    required String javaHome,
  }) {
    final sdkManagerExtension = Platform.isWindows ? '.bat' : '';
    final sdkManager = path.join(
      sdkPath,
      'cmdline-tools',
      'latest',
      'bin',
      'sdkmanager$sdkManagerExtension',
    );

    log.info('Installing NDK $ndkVersion');
    runCommand(sdkManager, [
      '--install',
      'ndk;$ndkVersion',
    ], environment: {
      'JAVA_HOME': javaHome,
    });
  }

  Future<Map<String, String>> buildEnvironment() async {
    // Load optional developer overrides from app/.env
    final dotEnv = _loadDotEnv();
    final effectiveSdkPath = dotEnv['ANDROID_SDK_ROOT'] ?? sdkPath;
    final effectiveNdkVersion = dotEnv['ANDROID_NDK_VERSION'] ?? ndkVersion;

    final hostArch = Platform.isMacOS
        ? "darwin-x86_64"
        : (Platform.isLinux ? "linux-x86_64" : "windows-x86_64");

    final ndkPath = path.join(effectiveSdkPath, 'ndk', effectiveNdkVersion);
    final toolchainPath = path.join(
      ndkPath,
      'toolchains',
      'llvm',
      'prebuilt',
      hostArch,
      'bin',
    );

    final minSdkVersion =
        math.max(target.androidMinSdkVersion!, this.minSdkVersion);

    final exe = Platform.isWindows ? '.exe' : '';

    final arKey = 'AR_${target.rust}';
    final arValue = ['${target.rust}-ar', 'llvm-ar', 'llvm-ar.exe']
        .map((e) => path.join(toolchainPath, e))
        .firstWhereOrNull((element) => File(element).existsSync());
    if (arValue == null) {
      throw Exception('Failed to find ar for $target in $toolchainPath');
    }

    String wrapperPrefix = target.rust;
    if (wrapperPrefix == 'armv7-linux-androideabi') {
      wrapperPrefix = 'armv7a-linux-androideabi';
    }
    // On Windows the NDK ships .cmd shims; on Linux/macOS they're plain binaries.
    final cmdExt = Platform.isWindows ? '.cmd' : '';
    final wrapperScript = path.join(toolchainPath, '$wrapperPrefix$minSdkVersion-clang$cmdExt');
    final wrapperScriptCxx = path.join(toolchainPath, '$wrapperPrefix$minSdkVersion-clang++$cmdExt');

    final ccKey = 'CC_${target.rust}';
    final cxxKey = 'CXX_${target.rust}';

    final linkerKey =
        'cargo_target_${target.rust.replaceAll('-', '_')}_linker'.toUpperCase();

    // Build BINDGEN_EXTRA_CLANG_ARGS so bindgen finds NDK headers on all platforms.
    final bindgenArgsKey = 'BINDGEN_EXTRA_CLANG_ARGS_${target.rust.replaceAll('-', '_')}';
    final sysrootPath = path.join(toolchainPath, '..', 'sysroot');
    // Clang requires forward slashes in --sysroot even on Windows.
    final sysrootPathNormalized = sysrootPath.replaceAll('\\', '/');
    var bindgenArgsValue = '--sysroot=$sysrootPathNormalized --target=$wrapperPrefix$minSdkVersion';

    // On Windows, libclang does not auto-discover the NDK clang resource
    // directory, so we must inject the compiler headers explicitly.
    if (Platform.isWindows) {
      final clangLibPath = path.join(toolchainPath, '..', 'lib', 'clang');
      if (Directory(clangLibPath).existsSync()) {
        final dirs = Directory(clangLibPath).listSync().whereType<Directory>().toList();
        if (dirs.isNotEmpty) {
          final clangIncludePath = path.join(dirs.first.path, 'include').replaceAll('\\', '/');
          bindgenArgsValue += ' -I$clangIncludePath';
        }
      }
    }

    final ranlibKey = 'RANLIB_${target.rust}';
    final ranlibValue = path.join(toolchainPath, 'llvm-ranlib$exe');

    final ndkVersionParsed = Version.parse(effectiveNdkVersion);
    final rustFlagsKey = 'CARGO_ENCODED_RUSTFLAGS';
    final rustFlagsValue = _libGccWorkaround(targetTempDir, ndkVersionParsed);

    final toolTempDir =
        Platform.environment['CARGOKIT_TOOL_TEMP_DIR'] ?? targetTempDir;

    return {
      arKey: arValue,
      ccKey: wrapperScript,
      cxxKey: wrapperScriptCxx,
      bindgenArgsKey: bindgenArgsValue,
      ranlibKey: ranlibValue,
      rustFlagsKey: rustFlagsValue,
      linkerKey: wrapperScript,
      // On Linux/macOS, CMake picks Ninja or Makefiles automatically.
      // On Windows it defaults to MSVC which cannot cross-compile for Android.
      if (Platform.isWindows) 'CMAKE_GENERATOR': 'Ninja',
      'CARGOKIT_TOOL_TEMP_DIR': toolTempDir,
      'ANDROID_NDK_HOME': ndkPath,
    };
  }

  // Workaround for libgcc missing in NDK23, inspired by cargo-ndk
  String _libGccWorkaround(String buildDir, Version ndkVersion) {
    final workaroundDir = path.join(
      buildDir,
      'cargokit',
      'libgcc_workaround',
      '${ndkVersion.major}',
    );
    Directory(workaroundDir).createSync(recursive: true);
    if (ndkVersion.major >= 23) {
      File(path.join(workaroundDir, 'libgcc.a'))
          .writeAsStringSync('INPUT(-lunwind)');
    } else {
      // Other way around, untested, forward libgcc.a from libunwind once Rust
      // gets updated for NDK23+.
      File(path.join(workaroundDir, 'libunwind.a'))
          .writeAsStringSync('INPUT(-lgcc)');
    }

    var rustFlags = Platform.environment['CARGO_ENCODED_RUSTFLAGS'] ?? '';
    if (rustFlags.isNotEmpty) {
      rustFlags = '$rustFlags\x1f';
    }
    rustFlags = '$rustFlags-L\x1f$workaroundDir';
    return rustFlags;
  }
}
