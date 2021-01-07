import falcon
import pyBack.settings

class Default:

    def on_get(self, req, resp):
        resp.body = 'Hello World'
        resp.status = falcon.HTTP_200

api = falcon.API()
api.add_route('/hello', Default())
