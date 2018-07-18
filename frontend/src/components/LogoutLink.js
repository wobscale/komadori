import React from 'react';
import PropTypes from 'prop-types';

const LogoutLink = props => (
  <div className="Logout">
    <a href="#" onClick={props.onClick}>Log out</a>
  </div>
);
LogoutLink.propTypes = {
  onClick: PropTypes.func.isRequired,
};

export default LogoutLink;
