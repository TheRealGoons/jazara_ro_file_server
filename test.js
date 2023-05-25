function dance(callback) {
	const moves = ['twist', 'shake', 'waggle'];
	// gettin ga random move
	let randomMove = moves[Math.floor(Math.random() * moves.length)];
	// get no move 50% of the time
	if (Math.random() > 0.5) randomMove = null;
	return callback(randomMove);
}

dance((tits) => {
	console.log(tits)
})
