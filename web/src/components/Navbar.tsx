import './Navbar.css'

function Navbar() {
  return (
    <header className="navbar">
      <a className="brand" href="/" tabIndex={-1}>
        chess
      </a>
      <button className="login-button" type="button">
        Login
      </button>
    </header>
  )
}

export default Navbar
