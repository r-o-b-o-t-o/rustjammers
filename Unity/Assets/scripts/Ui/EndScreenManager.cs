using System;
using Main;
using TMPro;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace Ui
{
	public class EndScreenManager : MonoBehaviour
	{
		// Author: Created by Esteban / Edited by Axel
		[Serializable]
		public enum WinState
		{
			Winner,
			Loser,
			Draw
		}

		[Serializable]
		private struct PlayerScores
		{
			[SerializeField] public TextMeshProUGUI StateText;
			[SerializeField] public TextMeshProUGUI ScoreText;
		}

		[SerializeField] private PlayerScores[] scores;
		[SerializeField] private GameViewManagerScript gameViewManager;
		[SerializeField] private GameObject ui;
		[SerializeField] private string mainMenuScene;

		private void SetScoreForPlayer(int index, int score, string state)
		{
			var scoreString = score.ToString();
			if (score < 10)
			{
				scoreString = "0" + scoreString;
			}
			this.scores[index].ScoreText.text = scoreString;
			this.scores[index].StateText.text = state;
		}

		public void SetScore(int p1, int p2)
		{
			var state1 = WinState.Draw;
			var state2 = WinState.Draw;

			if (p1 > p2)
			{
				state1 = WinState.Winner;
				state2 = WinState.Loser;
			} else if (p2 > p1)
			{
				state2 = WinState.Winner;
				state1 = WinState.Loser;
			}

			this.SetScoreForPlayer(0, p1, state1.ToString());
			this.SetScoreForPlayer(1, p2, state2.ToString());
		}

		public void OnPlayAgainClicked()
		{
			this.gameViewManager.PlayAgain();
		}

		public void OnBackToMenuClicked()
		{
			SceneManager.LoadScene(this.mainMenuScene, LoadSceneMode.Single);
		}

		public void Enable()
		{
			this.ui.SetActive(true);
		}

		public void Disable()
		{
			this.ui.SetActive(false);
		}
	}
}