import config from '../config';

class GithubAPI {
  static authorizeUrl() {
    return fetch(`${config.api}/github/authorize_url`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'GET',
    }).then((resp) => {
      if (resp.status >= 300) {
        throw new Error(resp.statusText);
      }
      return resp.text();
    });
  }
}

export default GithubAPI;
