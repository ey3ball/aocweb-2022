import { useRouteError } from "react-router-dom";

export default function ErrorPage() {
  const error = useRouteError();

  return (
    <>
    <div>
      <h1>Page not found</h1>
      {error.statusText || error.message}
    </div>
    </>
  )
}
