$pkg_name="hab-shell"
$pkg_version="0.1.12"

$pkg_deps=@(
    "core/openssl",
    "core/zlib",
    "core/libarchive",
    "core/libsodium",
    "core/xz",
    "core/hab"
)

$pkg_build_deps = @(
    "core/visual-cpp-build-tools-2015",
    "core/rust",
    "core/cacerts"
)

$pkg_bin_dirs = @("bin")

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    }
    else {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CACHE_SRC_PATH/$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:SODIUM_LIB_DIR             = "$(Get-HabPackagePath "libsodium")/lib"
    $env:LIBARCHIVE_INCLUDE_DIR     = "$(Get-HabPackagePath "libarchive")/include"
    $env:LIBARCHIVE_LIB_DIR         = "$(Get-HabPackagePath "libarchive")/lib"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$(Get-HabPackagePath "openssl")/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$(Get-HabPackagePath "openssl")/include"

}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    Copy-Item ../../Cargo.toml .
    Copy-Item ../../Cargo.lock .
    Copy-Item ../../src . -Recurse -Force
    cargo build --release --verbose
}

function Invoke-Install {
    Write-BuildLine "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab-shell.exe" "$pkg_prefix/bin/hab-shell.exe"
    # Copy-Item "$(Get-HabPackagePath "openssl")/bin/*.dll" "$pkg_prefix/bin"
    # Copy-Item "$(Get-HabPackagePath "zlib")/bin/*.dll" "$pkg_prefix/bin"
    # Copy-Item "$(Get-HabPackagePath "libarchive")/bin/*.dll" "$pkg_prefix/bin"
    # Copy-Item "$(Get-HabPackagePath "libsodium")/bin/*.dll" "$pkg_prefix/bin"
    # Copy-Item "$(Get-HabPackagePath "xz")/bin/*.dll" "$pkg_prefix/bin"
    # Copy-Item "$(Get-HabPackagePath "visual-cpp-build-tools-2015")/Program Files/Microsoft Visual Studio 14.0/VC/redist/x64/Microsoft.VC140.CRT/*.dll" "$pkg_prefix/bin"
}


function _install-dependency($dependency) {
    If (-Not $(hab pkg path $dependency)) {
	if (!$env:NO_INSTALL_DEPS) {
	    & $HAB_BIN install -u $env:HAB_BLDR_URL --channel $env:HAB_BLDR_CHANNEL $dependency
	    if ($LASTEXITCODE -ne 0 -and ($env:HAB_BLDR_URL -ne $FALLBACK_CHANNEL)) {
		Write-BuildLine "Trying to install '$dependency' from '$FALLBACK_CHANNEL'"
		& $HAB_BIN install -u $env:HAB_BLDR_URL --channel $FALLBACK_CHANNEL $dependency
	    }
	}
    }
}
