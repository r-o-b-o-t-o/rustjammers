using UnityEngine;

namespace Main
{
	public class AgentTypeScript : MonoBehaviour
	{
		// Author: Created by Esteban / Edited by Axel
		public enum AgentType
		{
			Human = 0,
			Random = 1,
			Rollout = 2,
			Dijkstra = 3,
			QLearning = 4
		}

		public static AgentTypeScript Instance;

		[HideInInspector]
		public AgentType[] Types;

		private void Start()
		{
			Instance = this;
			DontDestroyOnLoad(this.gameObject);
			this.Types = new AgentType[2];
		}
	}
}
