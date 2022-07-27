FROM gitpod/workspace-rust

# Install trunk
RUN bash -cl "wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-"
RUN bash -cl "sudo mv ./trunk /usr/bin/"

# Install wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown
