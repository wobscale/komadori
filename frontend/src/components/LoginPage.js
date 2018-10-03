import React from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import logo from '../logo.svg';

const LoginPage = (props) => {
  const loginLinks = props.providers.map(p => (
    <Link key={p.name} to={{ pathname: p.pathname, search: props.location.search }}>
      <button type="button">
        Login with {p.name}
      </button>
    </Link>
  ));

  return (
    <div className="WobscaleAccounts">
      <header className="Wobscale-Login">
        <img src={logo} className="Wobscale-logo" alt="logo" />
      </header>
      <h1 className="App-title">Login</h1>
      <p>
        Login to your Wobscale Account
      </p>
      {loginLinks}
      <h2>Create an account</h2>
      <p> If you {"don't"} have an account yet, login to create one </p>
    </div>
  );
};
LoginPage.propTypes = {
  location: PropTypes.object.isRequired,
  providers: PropTypes.object.isRequired,
};

export default LoginPage;
