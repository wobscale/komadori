import config from './config';

class UserAPI {
  static get() {
    return fetch(`${config.api}/user`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'GET',
    }).then((resp) => {
      if (resp.status === 404) {
        return { loggedIn: false };
      }
      return resp.json().then((user) => {
        if (user.Ok) {
          return {
            loggedIn: true,
            user: user.Ok,
          };
        }
        if (user.Err) {
          throw new Error('api error: ', user.Err);
        }
        throw new Error('error: api gave back invalid json: ', user);
      });
    });
  }
}

export default UserAPI;
