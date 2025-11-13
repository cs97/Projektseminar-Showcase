# setup backend
```
podman run --name Projektseminar-Showcase-database -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD=MLUISCOOL -d postgres
```

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

```
diesel setup
```

```
git clone git@github.com:cs97/Projektseminar-Showcase.git
```

```
cd backend
```

```
cargo run
```

# setup frontend
```
cd frontend
npm install
```

start on local server
```
vite
```

build project (built project will be in "./dist")
```
npm run build
```
