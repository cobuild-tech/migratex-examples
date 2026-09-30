import { Navigate, Outlet } from 'react-router-dom';
import { useSession } from '../session/SessionContext';

export function RequireAuth() {
  const { isLoggedIn } = useSession();
  return isLoggedIn ? <Outlet /> : <Navigate to="/login" replace />;
}
