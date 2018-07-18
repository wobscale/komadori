import React from 'react';
import PropTypes from 'prop-types';
import LogoutContainer from '../containers/LogoutContainer';

class Topbar extends React.Component {
  constructor(props) {
    super(props);
    this.state = { menuActive: false };

    this.toggleMenu = this.toggleMenu.bind(this);
  }

  toggleMenu() {
    this.setState(prevState => ({ menuActive: !prevState.menuActive }));
  }

  render() {
    if (this.props.user) {
      return (
        <div className="topbar">
          <ul role="navigation">
            <li className="center-topbar">Wobscale Account System</li>
            <div className={`dropdown ${this.state.menuActive ? ' is-active' : ''}`}>
              <div className="dropdown-trigger">
                <button
                  className="dropdown-button"
                  aria-haspopup="true"
                  aria-controls="dropdown-menu"
                  onClick={this.toggleMenu}
                >
                  <span>
                    {this.props.user.username}
                    <i className="fas fa-angle-down" aria-hidden="true" />
                  </span>
                </button>
              </div>
              <div className="dropdown-menu" id="dropdown-menu" role="menu">
                <div className="dropdown-content">
                  <LogoutContainer />
                </div>
              </div>
            </div>
          </ul>
        </div>
      );
    }
    return (
      <div id="topbar">
        <a href="/">Log in or Sign Up</a>
      </div>
    );
  }
}
Topbar.propTypes = {
  user: PropTypes.object,
};
Topbar.defaultProps = {
  user: null,
};

export default Topbar;
