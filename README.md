# openrouter-toolkit

Compile-time checked OpenRouter model ids.

`model_supports!` validates a model id and its required capabilities against a vendored OpenRouter index, then expands to the model id string.

```rs
use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!(
    "openai/gpt-5.4",
    param::tools,
    input::image,
    output::text,
);
```

Dynamic variants work too:

```rs
const MODEL: &str = model_supports!("moonshotai/kimi-k2-0905:exacto", param::tools);
```

## Capabilities

- `param::*` — request parameters (e.g. `param::tools`)
- `input::*` — input modalities (e.g. `input::image`)
- `output::*` — output modalities (e.g. `output::text`)

## Errors at compile time

Unknown capability:

```rs
const MODEL: &str = model_supports!("qwen/qwen3.7-max", param::toolz);
```

```text
error: unknown OpenRouter capability `param::toolz`; did you mean `param::tools`?
```

Capability not supported by the model:

```rs
const MODEL: &str = model_supports!("qwen/qwen3.7-max", input::image);
```

```text
error: OpenRouter model `qwen/qwen3.7-max` does not support required capability(s): input::image
```
