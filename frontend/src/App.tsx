import { useEffect, useState } from 'react'
import type { User } from './bindings/User'

function App() {
  const [user, setUser] = useState<User | null>(null)

  useEffect(() => {
    fetch('/api/user')
      .then(r => r.json())
      .then(setUser)
  }, [])

  return <pre>{JSON.stringify(user, null, 2)}</pre>
}

export default App