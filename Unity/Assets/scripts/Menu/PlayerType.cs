
using System;

using UnityEngine;
using UnityEngine.UI;
//Esteban
public class PlayerType : MonoBehaviour
{
	public enum AgentType {
		Random = 0,
		Human = 1,
		Rollout = 2,
		Dijkstra = 3,
		TabularQLearning = 4
	}

	public static PlayerType MyPlayersType;
	public AgentType typeP1;
	public AgentType typeP2;
	public Text typeP1txt;
	public Text typeP2txt;
	
	void Start ()
	{
		MyPlayersType = this;
		typeP1 = AgentType.Human;
		typeP2 = AgentType.Random;
	}
}
