# Steps to reproduce

Install `uv` (https://docs.astral.sh/uv/getting-started/installation/), then:

```shell
cd host
uv run maturin develop
```

See the LICENSE file disappear.

In `pyproject.toml` change Maturin version to 1.8 and see that it is not deleted.