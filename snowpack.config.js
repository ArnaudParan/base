const PORT = Number(process.env.SPORT) ||  8080
const HOST = process.env.SHOST ||  'localhost'
const PROXY_PY = process.env.PROXY_PY
const PROXY_R = process.env.PROXY_R

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
    port: PORT,
    hostname: HOST,
    output: 'stream',
  },
  mount: {
    'public': { url: '/' },
    'js': { url: '/js/' },
    'elm': { url: '/elm/' },
  },
  proxy: {
    '/apip': PROXY_PY,
    '/apir': PROXY_R
  },
};
