using System;
using System.Collections.Generic;
using System.Linq;
using Main;
using TMPro;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

namespace Ui
{
	public class MainMenu : MonoBehaviour
	{
		[SerializeField] private string gameScene;

		[SerializeField] private TMP_Dropdown[] agentDropdowns;
		[SerializeField] private GameObject[] agentKbds;
		public InputField[] inputFields;
		[SerializeField] private AgentTypeScript agentTypeManager;

		private void Start()
		{
			var names = Enum.GetNames(typeof(AgentTypeScript.AgentType));
			var list = (from n in names where n != AgentTypeScript.AgentType.QLearning.ToString() select new TMP_Dropdown.OptionData(n)).ToList();
			foreach (var dropdown in this.agentDropdowns)
			{
				dropdown.options = new List<TMP_Dropdown.OptionData>(list);
				dropdown.onValueChanged.Invoke(dropdown.value); // Forces OnPxAgentTypeChanged to run at startup
			}
			this.agentDropdowns[0].options.Add(new TMP_Dropdown.OptionData(AgentTypeScript.AgentType.QLearning.ToString()));
		}

		public void onP1NbFrameChanged()
		{
			var number = 500;
			var val = int.TryParse(inputFields[0].text, out number);
			if (val && number > 10)
			{
				agentTypeManager.nbFrames1 = number;
			} else
			{
				agentTypeManager.nbFrames1 = 10;
			}
			inputFields[0].text = "" + agentTypeManager.nbFrames1;
		}

		public void onP2NbFrameChanged()
		{
			var number = 500;
			var val = int.TryParse(inputFields[1].text, out number);
			if (val && number > 10)
			{
				agentTypeManager.nbFrames2 = number;
			} else
			{
				agentTypeManager.nbFrames2 = 10;
			}
			inputFields[1].text = "" + agentTypeManager.nbFrames2;
		}

		public void onP1NbSimChanged()
		{
			var number = 500;
			var val = int.TryParse(inputFields[2].text, out number);
			if (val && number > 0)
			{
				agentTypeManager.nbSim1 = number;
			} else
			{
				agentTypeManager.nbSim1 = 1;
			}
			inputFields[2].text = "" + agentTypeManager.nbSim1;
		}

		public void onP2NbSimChanged()
		{
			var number = 500;
			var val = int.TryParse(inputFields[3].text, out number);
			if (val && number > 0)
			{
				agentTypeManager.nbSim2 = number;
			} else
			{
				agentTypeManager.nbSim2 = 1;
			}
			inputFields[3].text = "" + agentTypeManager.nbSim2;
		}

		public void OnPlayButtonClicked()
		{
			for (var i = 0; i < this.agentDropdowns.Length; i++)
			{
				this.agentTypeManager.Types[i] = (AgentTypeScript.AgentType) this.agentDropdowns[i].value;
			}
			SceneManager.LoadSceneAsync(this.gameScene, LoadSceneMode.Single);
		}

		public void OnTurboButtonClicked()
		{
			for (var i = 0; i < this.agentDropdowns.Length; i++)
			{
				this.agentTypeManager.Types[i] = (AgentTypeScript.AgentType) this.agentDropdowns[i].value;
			}
			SceneManager.LoadSceneAsync(this.gameScene, LoadSceneMode.Single);
			AgentTypeScript.Instance.turbo = true;
		}

		public void OnQuitButtonClicked()
		{
#if UNITY_EDITOR
			UnityEditor.EditorApplication.isPlaying = false;
#else
			Application.Quit();
#endif
		}

		public void OnP1AgentTypeChanged(Int32 val)
		{
			var human = val == (Int32) AgentTypeScript.AgentType.Human;
			var rollout = val == (Int32) AgentTypeScript.AgentType.Rollout;
			this.agentKbds[0].SetActive(human);
			this.inputFields[0].gameObject.SetActive(rollout);
			this.inputFields[2].gameObject.SetActive(rollout);
		}

		public void OnP2AgentTypeChanged(Int32 val)
		{
			var human = val == (Int32) AgentTypeScript.AgentType.Human;
			var rollout = val == (Int32) AgentTypeScript.AgentType.Rollout;
			this.agentKbds[1].SetActive(human);
			this.inputFields[1].gameObject.SetActive(rollout);
			this.inputFields[3].gameObject.SetActive(rollout);
		}
	}
}