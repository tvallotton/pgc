FROM rust
RUN curl -fsSL https://deno.land/install.sh | sh
WORKDIR /home/app
COPY . .
RUN ENV=prod /root/.deno/bin/deno compile --target x86_64-apple-darwin --allow-sys --allow-net --allow-run --allow-env --allow-write --allow-read --output pgc-x86_64-apple-darwin src/main.ts
RUN ENV=prod /root/.deno/bin/deno compile --target x86_64-unknown-linux-gnu --allow-sys --allow-net --allow-run --allow-env --allow-write --allow-read --output pgc-x86_64-unknown-linux-gnu src/main.ts
RUN ENV=prod /root/.deno/bin/deno compile --target aarch64-unknown-linux-gnu --allow-sys --allow-net --allow-run --allow-env --allow-write --allow-read --output pgc-aarch64-unknown-linux-gnu src/main.ts
