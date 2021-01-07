module.exports = {
  plugins: [
    [
      '@snowpack/plugin-webpack',
      {},
    ],
  ],
  buildOptions: {
    baseUrl: '/static',
    clean: true,
    webModulesUrl: 'web_modules',
    metaDir: '__snowpack__',
  },
  devOptions: {
    open: 'false',
  },
  mount: {
    'public': { url: '/' },
    'frontend': { url: '/' },
    'config/js': { url: '/' },
  },
  proxy: {
    '/apip': 'http://localhost:8000/apip',
    '/apir': 'http://localhost:8000/apir',
  },
};
