import React from 'react';
import { Link } from 'react-router-dom';
import logo from '../logo.svg';

const LoginPage = (props) => (
  <div className="WobscaleAccounts">
    <header className="Wobscale-Login">
      <img src={logo} className="Wobscale-logo" alt="logo" />
    </header>
    <h1 className="App-title">Login</h1>
    <p>
      Login to your Wobscale Account
    </p>
    <Link to={{ pathname: '/github/login', search: props.location.search }}>
      <button type="button">
        Login with Github
      </button>
    </Link>

    <h2>Create an account</h2>
    <p> If you {"don't"} have an account yet, login to create one </p>
  </div>
);

export default LoginPage;
