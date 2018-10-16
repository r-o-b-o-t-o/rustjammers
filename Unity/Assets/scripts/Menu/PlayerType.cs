
using System;

using UnityEngine;
using UnityEngine.UI;
//Esteban
public class PlayerType : MonoBehaviour
{

	/*Random
	 *Human
	 *Rollout
	 *Dijkstra
	 *TabularQLearning
	 */

	public static PlayerType MyPlayersType;
	public sbyte typeP1;
	public sbyte typeP2;
	public Text typeP1txt;
	public Text typeP2txt;
	
	void Start ()
	{
		MyPlayersType = this;
		typeP1 = 1;
		typeP2 = 0;
	}
}
