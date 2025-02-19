# BYOS_rs

This is a Rust implementation of a self-hostable server implementation for TRMNL displays.

It is still early days and more a first draft.
## Currently supported plugins

- Github Commit Graph

more to come...

## Configuration
A schedule is configurable with a config file. eg:

```yaml
default_screen:
  plugin_type: githubcommitgraph

schedules:
  - screen: "GithubDay"
    start_time: "08:00"
    end_time: "16:00"
    update_interval: 600
    days: ["Monday", "Wednesday", "Friday"]
    plugin:
      !githubcommitgraph
  - screen: "GithubNight"
    start_time: "16:00"
    end_time: "23:00"
    update_interval: 600
    days: ["Monday", "Wednesday", "Friday"]
    plugin:
      !staticimage
        path: "./data/peter.bmp"
  - screen: "Screen3"
    start_time: "23:00"
    end_time: "08:00"
    update_interval: 600
    days: ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
    plugin:
      !custom
          template: |
            <div>
              <h1>Welcome to Screen 3</h1>
              <p>This is a custom HTML template for Screen 3.</p>
            </div>
          plugin_code: |  # Ruby content code
            def custom_content
              # Your Ruby code here
              "This content is generated by Ruby code."
            end

plugin_config:
  githubcommitgraph:
    username: "lukas-heiligenbrunner"
    api_key: "<your_key>"
```

If schdules overlap the first time match is used.

## Features missing:
- More Plugins
- custom code plugin support
- trmnl authentication with token
- multiple trmnl devices

## License

This project is licensed under the MIT License. Feel free to contribute and modify as per the guidelines outlined in the license agreement.


docker run --env SERVER_URL=http://192.168.0.194:8080 --env CONFIG_PATH=/test.yaml -p 8080:8080 -v ./data/test.yaml:/test.yaml b1595f86f1603120d8497c9a3fa394f3bb4bf4531c7975245076dc6b6c86d1a8
