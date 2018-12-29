$pkg_name="hab-shell-repo"
$pkg_version="1.0.52"

$pkg_deps=@(
    "core/hab",
    "core/powershell"
)

function Invoke-Shell {
    echo "hello from pwsh!"
}

function Invoke-Build {
}

function Invoke-Install {
}
