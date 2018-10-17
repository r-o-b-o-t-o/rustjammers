using System;
using System.Collections.Generic;
using Main;
using TMPro;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace Ui
{
	public class MainMenu : MonoBehaviour
	{
		[SerializeField] private string gameScene;
		[SerializeField] private TMP_Dropdown[] agentDropdowns;
		[SerializeField] private GameObject[] agentKbds;
		[SerializeField] private AgentTypeScript agentTypeManager;

		private void Start()
		{
			var list = new List<TMP_Dropdown.OptionData>();
			var names = Enum.GetNames(typeof(AgentTypeScript.AgentType));
			foreach (var n in names)
			{
				list.Add(new TMP_Dropdown.OptionData(n));
			}
			foreach (var dropdown in this.agentDropdowns)
			{
				dropdown.options = list;
				dropdown.onValueChanged.Invoke(dropdown.value); // Forces OnPxAgentTypeChanged to run at startup
			}
		}

		public void OnPlayButtonClicked()
		{
			for (var i = 0; i < this.agentDropdowns.Length; i++)
			{
				this.agentTypeManager.Types[i] = (AgentTypeScript.AgentType) this.agentDropdowns[i].value;
			}
			SceneManager.LoadSceneAsync(this.gameScene, LoadSceneMode.Single);
		}

		public void OnP1AgentTypeChanged(Int32 val)
		{
			var human = val == (Int32) AgentTypeScript.AgentType.Human;
			this.agentKbds[0].SetActive(human);
		}

		public void OnP2AgentTypeChanged(Int32 val)
		{
			var human = val == (Int32) AgentTypeScript.AgentType.Human;
			this.agentKbds[1].SetActive(human);
		}
	}
}
