# Taigla web app

## Developpement
Install tailwind dependencies
```bash
npm install
```

Open 2 terminal window
In the first one run:
```bash
export TAIGLA_BACKEND_URL=http://localhost:8000
dx serve --hot-reload --platform web --port 1234
```
And in the second one:
```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```
