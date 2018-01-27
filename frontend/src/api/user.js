import config from '../config';

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

  static logout() {
    return fetch(`${config.api}/user/logout`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'GET',
    }).then((resp) => {
      if (resp.ok) {
        return {};
      }
      throw new Error(`could not logout: ${resp.status}`);
    });
  }

  static auth(provider, code, state) {
    return fetch(`${config.api}/user/auth`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'POST',
      body: JSON.stringify({
        provider,
        code,
        state,
      }),
    }).then((resp) => {
      if (!resp.ok) {
        // TODO: parse out error
        throw new Error(`auth error: ${resp.status}`);
      }
      return resp.json().then((authRes) => {
        if (authRes.Ok) {
          return authRes.Ok;
        }

        throw new Error('Response did not include a user or partial user.');
      });
    });
  }
}

export default UserAPI;
