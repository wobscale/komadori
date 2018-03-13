import { connect } from 'react-redux';
import UserDashboard from '../components/UserDashboard';

const mapStateToProps = (state) => {
  const { user } = state;
  return {
    user: user.user,
  };
};

const UserDashboardContainer = connect(
  mapStateToProps,
  null,
)(UserDashboard);

export default UserDashboardContainer;
