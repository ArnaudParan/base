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
        debugger: (process.env.NODE_ENV === 'development') ? 'dev' : 'never',
        optimize: (process.env.NODE_ENV === 'production') ? 'build' : 'never',
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
    'frontend': { url: '/frontend/' },
    'elm-front': { url: '/elm-front/' },
  },
  proxy: {
    '/apip': 'http://localhost:8000/apip',
    '/apir': 'http://localhost:8000/apir',
  },
};
