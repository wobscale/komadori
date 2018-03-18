import React from 'react';
import PropTypes from 'prop-types';

const LogoutLink = props => (
  <div className="Logout">
    <button onClick={props.onClick}>Log out</button>
  </div>
);
LogoutLink.propTypes = {
  onClick: PropTypes.func.isRequired,
};

export default LogoutLink;
