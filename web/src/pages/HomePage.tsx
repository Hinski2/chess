import './HomePage.css'

function HomePage() {
  return (
    <main className="welcome-main">
      <section className="welcome-content">
        <p className="eyebrow">YOUR NEXT MOVE</p>
        <h1 id="welcome-title">Ready to play?</h1>
        <p className="welcome-description">
          Choose how you want to start your game.
        </p>

        <div className="game-options">
          <button className="game-option game-option-primary" type="button">
            <span className="game-option-title">Play with bot</span>
            <span className="game-option-description">
              Challenge the computer
            </span>
            <span className="game-option-arrow">
              ↗
            </span>
          </button>
          <button className="game-option" type="button">
            <span className="game-option-title">Play online</span>
            <span className="game-option-description">Find an opponent</span>
            <span className="game-option-arrow">
              ↗
            </span>
          </button>
        </div>
      </section>
    </main>
  )
}

export default HomePage
