module.exports = {
  plugins: [
    [
      '@snowpack/plugin-webpack',
      {},
    ],
    [
      'snowpack-plugin-elm',
      {
        verbose: true,
        debugger: 'dev',
        optimize: 'build',
      },
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
    'js': { url: '/js/' },
    'elm': { url: '/elm/' },
  },
  proxy: {
    '/apip': 'http://localhost:8000/apip',
    '/apir': 'http://localhost:8000/apir',
  },
};
