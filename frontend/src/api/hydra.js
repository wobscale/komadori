import config from '../config';

class HydraAPI {
  static getConsent(id) {
    return fetch(`${config.api}/oauth/consent?id=${id}`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'GET',
    }).then(resp => resp.json()).then((resp) => {
      if (resp.Ok && resp.Ok.client) {
        return resp.Ok;
      }

      throw new Error('Invalid consent response, did not include client');
    });
  }

  static acceptConsent(id, scopes) {
    return fetch(`${config.api}/oauth/consent/accept`, {
      body: JSON.stringify({
        id,
        scopes,
      }),
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'POST',
    }).then(resp => resp.json()).then(resp => resp.Ok);
  }

  static rejectConsent(id, reason) {
    return fetch(`${config.api}/oauth/consent/reject`, {
      body: JSON.stringify({
        id,
        reason,
      }),
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'POST',
    }).then(resp => resp.json()).then(resp => resp.Ok);
  }
}

export default HydraAPI;
