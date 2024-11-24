# Docker fetch

Intended to be used as a preview for [fzf](ttps://github.com/junegunn/fzf)

# Example: Pick a container and shell into.

Add to your .bashrc.

```bash
dsh() {
  container=$(docker ps -q | fzf --header="Pick Container" --preview="docker inspect {} | docker-fetch")
  echo -e "\033[36mShell:\033[0m $container"
  docker exec -it $container bash
}
```

# Install

```bash
cargo install docker-fetch --locked
```
