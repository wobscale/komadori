import { connect } from 'react-redux';
import SecurityComponent from '../components/Security';

const mapStateToProps = (state) => {
  const { user } = state;
  return {
    user: user.user,
  };
};

const Security = connect(
  mapStateToProps,
  null,
)(SecurityComponent);

export default Security;
