module.exports = {
  apps: [{
    name: 'scour',
    script: './target/release/scour',
    args: '-p 10080',
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: '200M',
    env: {
      PORT: '10080',
    },
    error_file: './logs/scour-error.log',
    out_file: './logs/scour-out.log',
    log_date_format: 'YYYY-MM-DD HH:mm:ss',
  }]
}
