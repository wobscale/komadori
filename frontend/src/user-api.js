import config from './config';

class UserAPI {
  static get() {
    return fetch(`${config.api}/user`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'GET',
    }).then(resp => resp.json()).then((resp) => {
      if (resp.uuid) {
        return resp;
      }

      throw new Error('Invalid user response, did not include uuid');
    });
  }
}

export default UserAPI;
