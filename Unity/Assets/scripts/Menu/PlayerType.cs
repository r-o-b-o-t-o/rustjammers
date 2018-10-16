
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
	public int p1id;
	public int p2id;
	
	void Start ()
	{
		MyPlayersType = this;
		typeP1 = AgentType.Human;
		typeP2 = AgentType.Random;
		p1id = 1;
		p2id = 0;
	}

	public void NewNamep1(int id)
	{
		switch (id)
		{
			case 0:
				typeP1txt.text = "Random";
				typeP1 = PlayerType.AgentType.Random;
				break;
			case 1:
				typeP1txt.text = "Human";
				typeP1 = PlayerType.AgentType.Human;
				break;
			case 2:
				typeP1txt.text = "Rollout";
				typeP1 = PlayerType.AgentType.Rollout;
				break;
			case 3:
				typeP1txt.text = "Dijkstra";
				typeP1 = PlayerType.AgentType.Dijkstra;
				break;
			case 4:
				typeP1txt.text = "TabularQLearning";
				typeP1 = PlayerType.AgentType.TabularQLearning;
				break;
		}
		Debug.Log(typeP1);
	}
	
	public void NewNamep2(int id)
	{
		switch (id)
		{
			case 0:
				typeP2txt.text = "Random";
				typeP2 = PlayerType.AgentType.Random;
				break;
			case 1:
				typeP2txt.text = "Human";
				typeP2 = PlayerType.AgentType.Human;
				break;
			case 2:
				typeP2txt.text = "Rollout";
				typeP2 = PlayerType.AgentType.Rollout;
				break;
			case 3:
				typeP2txt.text = "Dijkstra";
				typeP2 = PlayerType.AgentType.Dijkstra;
				break;
			case 4:
				typeP2txt.text = "TabularQLearning";
				typeP2 = PlayerType.AgentType.TabularQLearning;
				break;
		}
		Debug.Log(typeP1);
	}
}
