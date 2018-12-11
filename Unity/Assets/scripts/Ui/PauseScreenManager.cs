using System;
using Main;
using TMPro;
using UnityEngine;
using UnityEngine.SceneManagement;


namespace Ui
{
	public class PauseScreenManager : MonoBehaviour
	{
		[SerializeField] public TextMeshProUGUI ScoreText1;
		[SerializeField] public TextMeshProUGUI ScoreText2;
		[SerializeField] private GameViewManagerScript gameViewManager;
		[SerializeField] private GameObject ui;
		[SerializeField] private string mainMenuScene;
		public bool isActived = false;

		public void SetScore(int p1, int p2)
		{
			var scoreString = p1.ToString();
			if (p1 < 10)
			{
				scoreString = "0" + scoreString;
			}
			ScoreText1.text = scoreString;
			scoreString = p2.ToString();
			if (p2 < 10)
			{
				scoreString = "0" + scoreString;
			}
			ScoreText2.text = scoreString;
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
			isActived = true;
			this.ui.SetActive(true);
		}

		public void Disable()
		{
			isActived = false;
			this.ui.SetActive(false);
		}
	}
}


