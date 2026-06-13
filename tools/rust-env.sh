#!/usr/bin/env bash
set -euo pipefail

for bin_dir in "$HOME/.cargo/bin" "/opt/homebrew/opt/rustup/bin" "/usr/local/opt/rustup/bin"; do
    if [[ -x "$bin_dir/rustup" ]]; then
        export PATH="$bin_dir:$PATH"
        break
    fi
done

if [[ "$(uname -s)" == "Darwin" ]] && command -v brew >/dev/null 2>&1; then
    llvm_prefix="$(brew --prefix llvm 2>/dev/null || true)"
    if [[ -n "$llvm_prefix" ]]; then
        export LIBCLANG_PATH="${LIBCLANG_PATH:-$llvm_prefix/lib}"

        clang_resource_dir=""
        for dir in "$llvm_prefix"/lib/clang/*; do
            if [[ -d "$dir/include" ]]; then
                clang_resource_dir="$dir"
            fi
        done

        sdk_path="$(xcrun --show-sdk-path 2>/dev/null || true)"
        if [[ -n "$sdk_path" ]]; then
            export SDKROOT="${SDKROOT:-$sdk_path}"
        fi

        if [[ -z "${BINDGEN_EXTRA_CLANG_ARGS:-}" ]]; then
            bindgen_args=()
            if [[ -n "$clang_resource_dir" ]]; then
                bindgen_args+=("-resource-dir" "$clang_resource_dir")
            fi
            if [[ -n "${sdk_path:-}" ]]; then
                bindgen_args+=("-isysroot" "$sdk_path")
            fi
            if [[ "${#bindgen_args[@]}" -gt 0 ]]; then
                export BINDGEN_EXTRA_CLANG_ARGS="${bindgen_args[*]}"
            fi
        fi
    fi
fi

exec "$@"
