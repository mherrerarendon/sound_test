import argparse
import subprocess
from shutil import copyfile
import os

script_dir = os.path.dirname(os.path.realpath(__file__))
root_dir = os.path.abspath(os.path.join(script_dir, '..'))


def bridge_codegen():
    rust_input = os.path.join(root_dir, 'tuner-rs', 'src', 'api.rs')
    dart_output = os.path.join(root_dir, 'lib', 'api.dart')
    c_output = os.path.join(root_dir, 'ios', 'Runner', 'api.h')
    cmd = ['flutter_rust_bridge_codegen', '--rust-input', rust_input,
           '--dart-output', dart_output, '--c-output', c_output]
    result = subprocess.run(cmd)
    return result.returncode


def build_rust(release):
    os.chdir(os.path.join(root_dir, 'tuner-rs'))
    cmd = ['cargo', 'build']
    if release:
        cmd.append('--release')
    build_return_code = subprocess.run(cmd).returncode

    cmd = ['cargo', 'lipo']
    if release:
        cmd.append('--release')
    lipo_return_code = subprocess.run(cmd).returncode

    os.chdir(root_dir)
    if build_return_code != 0 or lipo_return_code != 0:
        return 1
    return 0


def copy_artifacts(release):
    src_path = os.path.join(root_dir, 'tuner-rs', 'target', 'universal',
                            'release' if release else 'debug', 'libtuner_rs.a')
    dest_path = os.path.join(root_dir, 'ios', 'Runner', 'libtuner_rs.a')
    if os.path.exists(dest_path):
        os.remove(dest_path)
    copyfile(src_path, dest_path)


if __name__ == '__main__':
    print('Remember to set `crate-type = [\"staticlib\"]` in Cargo.toml')
    cli = argparse.ArgumentParser(
        description='Build script for sound test')
    cli.add_argument('-d', '--debug', action='store_true',
                     help='Set to build as release')
    args = cli.parse_args()

    if bridge_codegen() != 0:
        print('Failed to generate bridge code')
        exit(1)
    if build_rust(not args.debug) != 0:
        print('Failed to build rust')
        exit(1)

    copy_artifacts(not args.debug)
    print('Build complete')
