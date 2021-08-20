import falcon
import pyBack.settings
import logging

class Default:

    def on_get(self, req, resp):
        logging.getLogger(__name__).debug('request', extra={'request': req})
        resp.body = '{"ok": "Hello World"}'
        resp.status = falcon.HTTP_200


api = falcon.API()
api.add_route('/hello', Default())
