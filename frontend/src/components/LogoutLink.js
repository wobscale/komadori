import React from 'react';
import { Link } from 'react-router-dom';

const LogoutLink = () => (
  <div className="Logout">
    <Link to="/user/logout">
      <a href="#">Log out</a>
    </Link>
  </div>
);

export default LogoutLink;
