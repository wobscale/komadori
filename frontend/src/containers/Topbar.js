import { connect } from 'react-redux';
import Component from '../components/Topbar';

const mapStateToProps = (state) => {
  const { user } = state;
  return {
    user: user.user,
  };
};

const Topbar = connect(
  mapStateToProps,
  null,
)(Component);

export default Topbar;
