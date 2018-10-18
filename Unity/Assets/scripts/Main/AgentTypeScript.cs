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

		public int nbFrames1=1000;
		public int nbFrames2=1000;
		public int nbSim1=3;
		public int nbSim2=3;
		
		[HideInInspector]
		public AgentType[] Types;

		private void Start()
		{
			Instance = this;
			DontDestroyOnLoad(this.gameObject);
			this.Types = new AgentType[2];
			nbFrames1=1000;
			nbFrames2=1000;
			nbSim1=3;
			nbSim2=3;
		}
	}
}
