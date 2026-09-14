export RUSTUP_HOME=/opt/sv-rustup
export CARGO_HOME=/opt/sv-cargo
case ":$PATH:" in
  *:/opt/sv-cargo/bin:*) ;;
  *) export PATH="/opt/sv-cargo/bin:$PATH" ;;
esac
