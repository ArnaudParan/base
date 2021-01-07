module.exports = {
  plugins: [
    [
      '@snowpack/plugin-webpack',
      {},
    ],
  ],
  devOptions: {
    open: 'false',
  },
  config: {
    mount: {
      'public': { url: '/static/' },
      'frontend': { url: '/static/' },
    },
  },
  proxy: {
    '/apip': 'http://localhost:8000/apip',
    '/apir': 'http://localhost:8000/apir',
  },
};
