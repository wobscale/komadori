import React, { Component } from 'react';
import qs from 'query-string';
import PropTypes from 'prop-types';
import config from './config';

class GithubOauthWindow extends Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
    };
  }
  componentDidMount() {
    const q = qs.parse(this.props.location.search);
    if (q.error) {
      let error = `Error: ${q.error}`;
      if (q.error_description) {
        error = `${error}: ${q.error_description}`;
      }
      window.opener.github_response = {
        error,
      };
      window.close();
    }

    if (q.code) {
      window.opener.github_response = {
        error: null,
        code: q.code,
        state: q.state,
      };
      window.close();
    }
    fetch(`${config.api}/github/authorize_url`, { credentials: 'include' })
      .then((res) => {
        if (res.status > 300) {
          throw new Error(res.statusText);
        }
        return res.text();
      })
      .then((url) => {
        window.location = url;
      })
      .catch((err) => {
        this.setState({ error: err.toString() });
      });
  }

  render() {
    if (this.state.error) {
      return (
        <div>
          <p>An error occurred: {this.state.error}</p>
        </div>
      );
    }
    return (
      <p>Redirecting to Github Oauth Login</p>
    );
  }
}
GithubOauthWindow.propTypes = {
  location: PropTypes.object.isRequired,
};

export default GithubOauthWindow;
