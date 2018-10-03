import config from '../config';

class AdminAPI {
  static bootstrap(token) {
    return fetch(`${config.api}/admin/bootstrap`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
      body: JSON.stringify({
        token,
      }),
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin bootstrap error: ${resp.status}`);
      }
      return resp.json();
    }).then((resp) => {
      if (resp) {
        return {};
      } else if (resp.status) {
        throw new Error(`Error status: ${resp.message}`);
      } else {
        throw new Error('unrecognized status');
      }
    });
  }

  static listUsers() {
    return fetch(`${config.api}/admin/users`, {
      method: 'GET',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin listusers error: ${resp.status}`);
      }
      return resp.json();
    }).then(resp => resp.users);
  }
}

export default AdminAPI;
