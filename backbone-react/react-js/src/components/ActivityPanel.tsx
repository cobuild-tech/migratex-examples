import { useEvents } from '../hooks/queries';
import type { Request } from '../state/AppState';

export function ActivityPanel({ request }: { request: Request }) {
  const { data: events = [] } = useEvents(request);
  return (
    <table className="ui-table table-stroke">
      <thead>
        <tr>
          <th>Event</th>
          <th>Action</th>
          <th>Repository</th>
        </tr>
      </thead>
      <tbody>
        {events.map((event) => (
          <tr key={event.id}>
            <th>{event.type}</th>
            <td>{event.payload?.action}</td>
            <td>{event.repo?.name}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
