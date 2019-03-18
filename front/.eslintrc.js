module.exports = {
  root: true,
  parserOptions: {
    parser: 'babel-eslint',
    sourceType: 'module'
  },
  env: {
    node: true,
    browser: true
  },
  plugins: [
    'vue'
  ],
  extends: [
    'standard',
    'plugin:vue/recommended'
  ]
}
