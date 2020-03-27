# Tauri/Quasar - Proof of Concept

## Preconditions
```bash
yarn
```

## Development

Start the frontend in development mode (hot-code reloading, error reporting, etc.).

- Frontend: in terminal 1, run:
    ```bash
    quasar dev
    ```
- Backend: in terminal 2, run:
    ```
    yarn tauri dev
    ```

- To customize configuration see [Configuring quasar.conf.js](https://quasar.dev/quasar-cli/quasar-conf-js).

## Production build
    ```bash
    yarn quasar build
    yarn tauri build
    ```
